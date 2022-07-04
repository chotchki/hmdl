import 'bootstrap/dist/css/bootstrap.min.css';
import './App.css';
import NavigationSystem from './NavigationSystem.js';

//Configure useAxios hook
import { configure } from 'axios-hooks';
import Axios from 'axios';
const axios = Axios.create({
  baseURL: '/',
})
configure({ axios, cache: false })

function App() {
  return (
    <div containter="App">
      <NavigationSystem />
    </div>
  );
}

export default App;
