interface Color {
    r: number,
    g: number,
    b: number,
}

interface MongoID {
    $oid: string,
}

interface List {
    _id: MongoID
    owner_id: MongoID,
    title: string,
    color: Color,
}

interface Lists {
    lists: Array<List>,
}

interface ListPatch {
    _id: MongoID | null,
    title: string | null,
    color: Color | null,
}

export {Lists, List, MongoID, Color, ListPatch};