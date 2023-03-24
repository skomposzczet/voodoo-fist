import axios from 'axios';
import authHeader from "./auth-header";

class DataService {
    get_lists() {
        return axios.get('api/lists', { headers: authHeader() });
    }
}

export default new DataService();