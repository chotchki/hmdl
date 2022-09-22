import React from 'react';
import 'bootstrap/dist/css/bootstrap.min.css';
import './App.css';
import { Routes, Route } from 'react-router-dom';

import Authentication from './utility/authentication/AuthenticationForm';
import AuthenticationProvider from './utility/authentication/AuthenticationProvider';
import ClientGroup from './client_groups/ClientGroup';
import ClientGroups from './client_groups/ClientGroups.js';
import Clients from './clients/Clients.js';
import DomainGroup from './domain_groups/DomainGroup.js';
import DomainGroups from './domain_groups/DomainGroups.js';
import Domains from './domains/Domains.js';
import Health from './utility/health';
import Layout from './utility/layout.js';
import PostSetup from './utility/setup/post-setup';
import PreSetup from './utility/setup/pre-setup';
import ToastProvider from './utility/toaster/ToastProvider';
import Users from './users/Users';


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
      <AuthenticationProvider>
        <Routes>
          <Route path="/" element={<Health />} />
          <Route path="/authentication" element={<Authentication />} />
          <Route path="/pre-setup" element={<PreSetup />} />
          <Route path="/post-setup" element={<PostSetup />} />
          <Route element={<Layout />}>
            <Route path="/clients" element={<Clients />} />
            <Route path="/client-groups" element={<ClientGroups />} />
            <Route path="/client-groups/:group" element={<ClientGroup />} />
            <Route path="/domains" element={<Domains />} />
            <Route path="/domain-groups" element={<DomainGroups />} />
            <Route path="/domain-groups/:group" element={<DomainGroup />} />
            <Route path="/users" element={<Users />} />
          </Route>
        </Routes>
      </AuthenticationProvider>
    </ToastProvider>
  );
}

export default App;
