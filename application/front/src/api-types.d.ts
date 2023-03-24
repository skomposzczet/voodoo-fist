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
    _id: MongoID,
    title?: string | null,
    color?: Color | null,
}

interface TodoItem {
    _id: MongoID,
    list_id: MongoID,
    text: string,
    is_done: boolean,
}

interface TodoItemPatch {
    _id: MongoID,
    text?: string | null,
    is_done?: boolean | null,
}

export {Lists, List, MongoID, Color, ListPatch, TodoItem, TodoItemPatch};