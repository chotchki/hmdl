import React from 'react';
import 'bootstrap/dist/css/bootstrap.min.css';
import './App.css';
import { Routes, Route } from 'react-router-dom';

import Clients from './clients/Clients.js';
import ClientGroup from './client_groups/ClientGroup';
import ClientGroups from './client_groups/ClientGroups.js';
import Domains from './domains/Domains.js';
import DomainGroups from './domain_groups/DomainGroups.js';
import Health from './utility/health';
import Layout from './utility/layout.js';
import ToastProvider from './utility/toaster/ToastProvider';


// Configure useAxios hook
import { configure } from 'axios-hooks';
import Axios from 'axios';
const axios = Axios.create({
  baseURL: '/',
});
configure({ axios, cache: false });

function App() {
  return (
    <ToastProvider>
      <Routes>
        <Route path="/" element={<Health />} />
        <Route element={<Layout />}>
          <Route path="/clients" element={<Clients />} />
          <Route path="/client-groups" element={<ClientGroups />}>
            <Route path=":group" element={<ClientGroup />} />
          </Route>
          <Route path="/domains" element={<Domains />} />
          <Route path="/domain-groups" element={<DomainGroups />} />
        </Route>
      </Routes>
    </ToastProvider>
  );
}

export default App;
