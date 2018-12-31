import * as https from "https";
import * as mustache from "mustache";
import * as fs from "fs";
import * as im from "immutable";

const whitelist = new Set(["io.k8s.api.core.v1.Pod"]);
const propBlacklist = new Set([
    //
    // TODO: Remove these.
    //
    "spec",
    "continue",
    // "type",

    "awsElasticBlockStore",
    "azureDisk",
    "azureFile",
    "cephfs",
    "cinder",
    "flexVolume",
    "flocker",
    "gcePersistentDisk",
    "gitRepo",
    "glusterfs",
    "iscsi",
    "nfs",
    "photonPersistentDisk",
    "portworxVolume",
    "projected",
    "quobyte",
    "rbd",
    "scaleIO",
    "storageos",
    "vsphereVolume"
]);

const renaming = new Map([["type", "condition_type"]]);

class Group {
    constructor(
        public readonly group: string,
        public readonly versionMap: im.Map<string, Version>
    ) {}

    public versions(): Version[] {
        return Array.from(this.versionMap.values()).sort((a, b) =>
            a.version.localeCompare(b.version)
        );
    }
}

class Version {
    constructor(
        public readonly version: string,
        public readonly topLevelKindMap: im.Map<string, Kind>,
        public readonly kindMap: im.Map<string, Kind>
    ) {}

    public topLevelKinds(): Kind[] {
        return Array.from(this.topLevelKindMap.values()).sort((a, b) =>
            a.kind.localeCompare(b.kind)
        );
    }

    public kinds(): Kind[] {
        return Array.from(this.kindMap.values()).sort((a, b) => a.kind.localeCompare(b.kind));
    }
}

class Kind {
    constructor(
        public readonly kind: string,
        public readonly isTopLevel: boolean,
        public readonly propertyList: im.List<Property>
    ) {}

    public properties(): Property[] {
        return this.propertyList.sort((a, b) => a.name.localeCompare(b.name)).toArray();
    }
}

class Property {
    constructor(public readonly name: string, public readonly type: string) {}
}

function parseGvk(id: string): [string, string, string] {
    let group, version, kind: string;
    if (id.startsWith("io.k8s.api.")) {
        const type = id.replace(/^io\.k8s\.api\./, "");
        const split = type.split(".");
        kind = split[split.length - 1];
        version = split[split.length - 2];
        group = split.slice(0, split.length - 2).join(".");
    } else if (id.startsWith("io.k8s.apimachinery.pkg.apis.")) {
        const type = id.replace(/^io\.k8s\.apimachinery\.pkg\.apis\./, "");
        const split = type.split(".");
        kind = split[split.length - 1];
        version = split[split.length - 2];
        group = split.slice(0, split.length - 2).join(".");
    } else if (id.startsWith("io.k8s.apimachinery.pkg.api.")) {
        const type = id.replace(/^io\.k8s\.apimachinery\.pkg\.api\./, "");
        const split = type.split(".");
        kind = split[split.length - 1];
        version = split[split.length - 2];
        group = split.slice(0, split.length - 2).join(".");
    } else {
        throw Error(`Extension ${id} APIs not supported`);
    }
    return [group, version, kind];
}

function tryGetTransitiveRef(prop: any): string | null {
    let ref = prop["$ref"] || (prop.items && prop.items["$ref"]);
    if (ref === undefined) {
        return null;
    }
    ref = ref.replace(/^#\/definitions\//, "");

    switch (ref) {
        case "io.k8s.apimachinery.pkg.apis.meta.v1.Time":
        case "io.k8s.apimachinery.pkg.util.intstr.IntOrString":
        case "io.k8s.apimachinery.pkg.api.resource.Quantity":
            return null;
    }

    return ref;
}

function makeRustType(prop: any): string {
    function rustTypeNameFromRef(ref: string): string {
        ref = ref.replace(/^#\/definitions\//, "");

        switch (ref) {
            case "io.k8s.apimachinery.pkg.apis.meta.v1.Time":
            case "io.k8s.apimachinery.pkg.util.intstr.IntOrString":
            case "io.k8s.apimachinery.pkg.api.resource.Quantity":
                return "String";
        }

        const [g, v, k] = parseGvk(ref);
        return `${g}::${v}::${k}`;
    }

    function rustPrim(type: string): string {
        switch (type) {
            case "string":
                return "String";
            case "integer":
                return "i32";
            case "boolean":
                return "bool";
            default:
                return type;
        }
    }

    // Is complex reference type.
    if ("$ref" in prop) {
        return rustTypeNameFromRef(prop["$ref"]);
    }

    // Is array.
    if (prop.type == "array") {
        if ("$ref" in prop.items) {
            return `Vec<${rustTypeNameFromRef(prop.items["$ref"])}>`;
        }
        return `Vec<${rustPrim(prop.items.type)}>`;
    }

    if (prop.type == "object") {
        const addProps = rustPrim(prop.additionalProperties.type);
        return `HashMap<${addProps}, ${addProps}>`;
    }

    return rustPrim(prop.type);
}

function reachableTypes(defns: any) {
    function visit(currTypeSpec: any, acc: im.Map<string, any>): im.Map<string, any> {
        for (const propId of Object.keys(currTypeSpec.properties || [])) {
            if (propBlacklist.has(propId)) {
                continue;
            }
            const prop = currTypeSpec.properties[propId];
            const ref = tryGetTransitiveRef(prop);
            if (ref != null) {
                const typeSpec = defns[ref];
                acc = acc.set(ref, typeSpec);
                acc = visit(typeSpec, acc);
            }
        }
        return acc;
    }

    let reachableTypes = im.Map<string, any>();

    for (const typeId of Object.keys(defns)) {
        if (whitelist.has(typeId)) {
            const type = defns[typeId];
            reachableTypes = visit(type, reachableTypes.set(typeId, type));
        }
    }

    return reachableTypes;
}

type Groups = im.Map<string, im.Map<string, im.Map<string, any>>>;

function partitionGroups(reachable: im.Map<string, any>): Groups {
    const groups = reachable
        // [List of types] -> [Map group -> type]
        .groupBy((_, typeId) => {
            const [g] = parseGvk(typeId);
            return g;
        })
        .toMap()
        .map(group =>
            // [Map group -> type] -> [Map group -> version -> type]
            group
                .groupBy((_, typeId) => {
                    const v = parseGvk(typeId)[1];
                    return v;
                })
                .toMap()
                // [Map group -> version -> type] -> [Map group -> version -> kind -> type]
                .map(types =>
                    types.reduce((acc, typeSpec, typeId) => {
                        const k = parseGvk(typeId)[2];
                        return acc.set(k, typeSpec);
                    }, im.Map<string, any>())
                )
        );

    return groups;
}

function renderProperties(kind: any): im.List<Property> {
    const props = kind.properties || {};
    const required = im.Set<string>(kind.required || []).union(["apiVersion", "kind"]);
    return im
        .List(Object.keys(props))
        .filter(propName => !propBlacklist.has(propName))
        .map(propName => {
            const rustType = makeRustType(props[propName]);
            const fqRustType = required.has(propName) ? rustType : `Option<${rustType}>`;
            return new Property(
                <string>(renaming.has(propName) ? renaming.get(propName) : propName),
                fqRustType
            );
        });
}

function renderGroupsView(groups: Groups): im.Map<string, Group> {
    return groups.map((group, groupName) => {
        const version = group.map((version, versionName) => {
            const topLevels = version
                .filter(kind => "x-kubernetes-group-version-kind" in kind)
                .map((kind, kindName) => new Kind(kindName, true, renderProperties(kind)));
            const subtypes = version
                .filter(kind => !("x-kubernetes-group-version-kind" in kind))
                .map((kind, kindName) => {
                    return new Kind(kindName, false, renderProperties(kind));
                });
            return new Version(versionName, topLevels, subtypes);
        });
        return new Group(groupName, version);
    });
}

function writeTypes(spec: any) {
    const defns = spec["definitions"];

    const reachable = reachableTypes(defns);
    const groups = partitionGroups(reachable);
    const groupsView = renderGroupsView(groups);

    const view = {
        groups: Array.from(groupsView.values()).sort((a, b) => a.group.localeCompare(b.group))
    };
    const modRs = mustache.render(
        fs.readFileSync("templates/types_mod.rs.mustache").toString(),
        view
    );
    fs.writeFileSync("../../src/types/api.rs", modRs);
}

function writePaths(spec: any) {
    const paths = spec["paths"];

    for (const path of Object.keys(paths)) {
        const api = paths[path];
        for (const verb of Object.keys(api)) {
            const verbSpec = api[verb];
            console.log(`${path} ${verb}`);
        }
    }
}

https.get(
    "https://raw.githubusercontent.com/kubernetes/kubernetes/master/api/openapi-spec/swagger.json",
    response => {
        // Continuously update stream with data
        var body = "";
        response.on("data", function(d) {
            body += d;
        });
        response.on("end", function() {
            // Data reception is done, do whatever with it!
            var parsed = JSON.parse(body);
            writeTypes(parsed);
            writePaths(parsed);
        });
    }
);

// const spec = fs
//     .readFileSync(
//         "/Users/alex/go/src/github.com/pulumi/pulumi-kubernetes/pkg/gen/openapi-specs/swagger-v1.13.0.json"
//     )
//     .toString();

// generateTypes(JSON.parse(spec));
