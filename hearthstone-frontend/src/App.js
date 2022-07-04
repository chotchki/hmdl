import 'bootstrap/dist/css/bootstrap.min.css';
import './App.css';
import Container from "react-bootstrap/Container";
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
    <Container>
      <NavigationSystem />
    </Container>
  );
}

export default App;
