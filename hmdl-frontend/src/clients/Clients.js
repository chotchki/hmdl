import React from 'react';
import useAxios from 'axios-hooks';

import Alert from 'react-bootstrap/Alert';
import Spinner from 'react-bootstrap/Spinner';

import ClientRow from './ClientRow';

export function Clients() {
  const [{ data, error, loading }, executeGetClients] =
    useAxios({ url: '/api/clients', method: 'GET' });

  if (error) {
    return (
      <Alert key="danger" variant="danger">
        Error: {error.message}
      </Alert>
    );
  } else if (loading) {
    return (
      <Spinner animation="border" role="status">
        <span className="visually-hidden">Loading...</span>
      </Spinner>
    );
  } else {
    return (
      <table className="table">
        <thead>
          <tr>
            <th scope="col">Name</th>
            <th scope="col">Ip</th>
            <th scope="col">MAC</th>
            <th scope="col">Assign Group</th>
            <th scope="col">Remove</th>
          </tr>
        </thead>
        <tbody>
          {data.map((client) => (
            <ClientRow key={client.name} client={client} refresh={executeGetClients} />
          ))}
        </tbody>
      </table>
    );
  }
}

export default Clients;
