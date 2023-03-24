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

export {Lists, List, MongoID, Color};