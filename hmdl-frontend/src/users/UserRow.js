import React, { useState } from 'react';
import useAxios from 'axios-hooks';
import PropTypes from 'prop-types';

import Alert from 'react-bootstrap/Alert';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import InputGroup from 'react-bootstrap/InputGroup';
import Spinner from 'react-bootstrap/Spinner';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { solid } from '@fortawesome/fontawesome-svg-core/import.macro';

import Timestamp from '../utility/Timestamp';
import { useToast } from '../utility/toaster/ToastProvider';

export function UserRow(props) {
  const [role, setRole] = useState(props.user.role);
  const [{ data, error, loading }] = useAxios({
    url: '/api/roles',
    method: 'GET',
  });

  const [{ }, executePut] = useAxios(
    {
      url: '/api/users/' + props.user.display_name,
      method: 'PUT',
    },
    { manual: true },
  );

  const updateUser = (event) => {
    executePut({
      data: {
        display_name: props.user.display_name,
        id: props.user.id,
        keys: props.user.keys,
        role: role,
      },
    }).then(() => {
      addToastSuccess('Role ' + role + ' assigned to ' + props.user.display_name + ' successfully');
      props.refresh();
    }).catch((e) => {
      addToastAxiosError(e, 'Unable to update user');
    });
  };

  const [{ }, executeDel] = useAxios(
    {
      url: '/api/users/' + props.user.display_name,
      method: 'DELETE',
    },
    { manual: true },
  );

  const deleteUser = (event) => {
    executeDel().then(() => {
      addToastSuccess('User ' + props.user.display_name + ' deleted successfully');
      props.refresh();
    }).catch((e) => {
      addToastAxiosError(e, 'Unable to delete user.');
    });
  };

  if (error) {
    return (
      <tr>
        <td>
          <Alert key="danger" variant="danger">
            Error: {error.message}
          </Alert>
        </td>
      </tr>
    );
  } else if (loading) {
    return (
      <tr>
        <td>
          <Spinner animation="border" role="status">
            <span className="visually-hidden">Loading...</span>
          </Spinner>
        </td>
      </tr>
    );
  } else {
    return (
      <tr>
        <td>
          props.display_name
        </td>
        <td>
          <InputGroup>
            <Form.Select onChange={(event) => setRole(event.target.value)}>
              <option key={role} value={role}>Current: {role}</option>
              {data.map((role) => (
                <option key={role} value={role}>{role}</option>
              ))}
            </Form.Select>
            <Button variant="primary" onClick={() => updateUser()}>
              <FontAwesomeIcon icon={solid('floppy-disk')} />
            </Button>
          </InputGroup>
        </td>
        <td>
          <Button variant="danger" onClick={() => deleteUser()}>
            <FontAwesomeIcon icon={solid('trash-can')} />
          </Button>
        </td>
      </tr >
    );
  }
}

UserRow.propTypes = {
  user: PropTypes.shape({
    display_name: PropTypes.string.isRequired,
    id: PropTypes.string.isRequired,
    keys: PropTypes.any.isRequired,
    role: PropTypes.string.isRequired
  }).isRequired,
  refresh: PropTypes.func.isRequired
};

export default UserRow;