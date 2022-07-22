import React from 'react';
import useAxios from 'axios-hooks';

import Alert from 'react-bootstrap/Alert';
import Container from 'react-bootstrap/Container';
import { Link } from 'react-router-dom';
import Spinner from 'react-bootstrap/Spinner';
import Table from 'react-bootstrap/Table';

import AddClientGroup from './AddClientGroup.js';

export function ClientGroups() {
  const [{ data, error, loading }, executeGet] = useAxios('/api/client-groups', 'GET');

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
  } else if (data.length === 0) {
    return (
      <>
        <Container>
          <h1>Existing Groups</h1>
          <p>No groups exist, setup a new one below.</p>
        </Container>
        <Container>
          <h1>Add New Group</h1>
          <AddClientGroup refresh={executeGet} />
        </Container>
      </>
    );
  }
  return (
    <>
      <Container>
        <h1>Existing Groups</h1>
        <Table>
          <thead>
            <tr>
              <th>Name</th>
            </tr>
          </thead>
          <tbody>
            {data.map((group) => (
              <tr key={group}>
                <td><Link to={group}>{group}</Link></td>
              </tr>
            ))}
          </tbody>
        </Table>
      </Container>
      <Container>
        <h1>Add New Group</h1>
        <AddClientGroup refresh={executeGet} />
      </Container>
    </>
  );
}

export default ClientGroups;
