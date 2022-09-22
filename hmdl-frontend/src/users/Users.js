import React from 'react';
import useAxios from 'axios-hooks';
import Alert from 'react-bootstrap/Alert';
import Spinner from 'react-bootstrap/Spinner';
import UserRow from './UserRow';

export function Users() {
  const [{ data, error, loading }, refreshUsers] = useAxios({
    url: '/api/users',
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
            <th scope="col">Username</th>
            <th scope="col">Assign Role</th>
            <th scope="col">Remove</th>
          </tr>
        </thead>
        <tbody>
          {data && data.map((u) => (
            <UserRow key={u.id} user={u} refresh={refreshUsers} />
          ))}
        </tbody>
      </table>
    );
  }
}

export default Users;
