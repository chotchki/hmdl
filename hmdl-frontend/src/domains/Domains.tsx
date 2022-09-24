import React from 'react';
import useAxios from 'axios-hooks';

import Alert from 'react-bootstrap/Alert';
import Spinner from 'react-bootstrap/Spinner';

import DomainRow from './DomainRow';

export function Domains() {
  const [{ data, error, loading }, executeGetDomains] = useAxios({
    url: '/api/domains',
    method: 'GET',
  });

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
            <th scope="col">Last Seen</th>
            <th scope="col">Last Client</th>
            <th scope="col">Assign Group</th>
            <th scope="col">Remove</th>
          </tr>
        </thead>
        <tbody>
          {data.map((domain) => (
            <DomainRow key={domain.name} domain={domain} refresh={executeGetDomains} />
          ))}
        </tbody>
      </table>
    );
  }
}

export default Domains;
