import 'bootstrap/dist/css/bootstrap.min.css';
import './App.css';
import { Routes, Route } from "react-router-dom";

import Clients from './clients/Clients.js';
import ClientGroups from './client_groups/ClientGroups.js';
import Domains from './domains/Domains.js';
import DomainGroups from './domain_groups/DomainGroups.js';
import Health from './utility/health';
import Layout from "./utility/layout.js";


//Configure useAxios hook
import { configure } from 'axios-hooks';
import Axios from 'axios';
const axios = Axios.create({
  baseURL: '/',
})
configure({ axios, cache: false })

function App() {
  return (
    <Routes>
      <Route path="/" element={<Health />} />
      <Route element={<Layout />}>
        <Route path="/clients" element={<Clients />} />
        <Route path="/client-groups" element={<ClientGroups />} />
        <Route path="/domains" element={<Domains />} />
        <Route path="/domain-groups" element={<DomainGroups />} />
      </Route>
    </Routes>
  );
}

export default App;