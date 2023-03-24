import { ListPatch, MongoID, TodoItemNew, TodoItemPatch } from '@/api-types';
import axios from 'axios';
import authHeader from "./auth-header";

class DataService {
    get_lists() {
        return axios.get('/api/lists', { headers: authHeader() });
    }

    patch_list(patch: ListPatch) {
        return axios.patch('/api/list', patch, { headers: authHeader() });
    }

    delete(endpoint: string, id: MongoID) {
        return axios.delete('/api/' + endpoint, { headers: authHeader(), data: id });
    }

    new_list(title: string) { 
        return axios.post('/api/list', {title: title}, { headers: authHeader() })
    }

    get_items(list_id: MongoID) {
        return axios.post('/api/items', {$oid: list_id.$oid}, { headers: authHeader() })
    }

    patch_item(patch: TodoItemPatch) {
        return axios.patch('/api/item', patch, { headers: authHeader() })
    }

    new_item(item: TodoItemNew) {
        return axios.post('/api/item', item, { headers: authHeader() });
    }
}

export default new DataService();