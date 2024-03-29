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

import { useToast } from '../utility/toaster/ToastProvider';

export function ClientRow(props) {
  const { addToastAxiosError, addToastSuccess } = useToast();
  const [{ data, error, loading }] = useAxios({ url: '/api/client-groups', method: 'GET' });
  const [clientName, setClientName] = useState(props.client.name);
  const [groupName, setGroupName] = useState('');

  const [{ }, executePut] = useAxios(
    {
      url: '/api/clients/' + props.client.name,
      method: 'PUT',
    },
    { manual: true },
  );

  const [{ }, executeGroupPut] = useAxios(
    {
      url: '/api/clients/' + clientName + '/group',
      method: 'PUT',
    },
    { manual: true },
  );

  const updateClient = (event) => {
    executePut({
      data: {
        name: clientName,
        ip: props.client.ip,
        mac: props.client.mac,
      },
    }).then(() => {
      executeGroupPut({
        data: { new_group_name: groupName },
      }).then(() => {
        addToastSuccess('Client ' + clientName + ' assigned to ' + groupName + ' successfully');
        props.refresh();
      }).catch((e) => {
        addToastAxiosError(e, 'Unable to assign group.');
      });
    }).catch((e) => {
      addToastAxiosError(e, 'Unable to update client');
    });
  };

  const [{ }, executeDel] = useAxios(
    {
      url: '/api/clients/' + clientName,
      method: 'DELETE',
    },
    { manual: true },
  );

  const deleteClient = (event) => {
    executeDel().then(() => {
      addToastSuccess('Client ' + clientName + ' deleted successfully');
      props.refresh();
    }).catch((e) => {
      addToastAxiosError(e, 'Unable to delete client.');
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
        <td><Form.Control
          type="text"
          defaultValue={clientName}
          onChange={(event) => setClientName(event.target.value)} /></td>
        <td>{props.client.ip}</td>
        <td>{props.client.mac}</td>
        <td>
          <InputGroup>
            <Form.Select onChange={(event) => setGroupName(event.target.value)}>
              <option>Assign Group</option>
              {data.map((group) => (
                <option key={group}>{group}</option>
              ))}
            </Form.Select>
            <Button variant="primary" onClick={() => updateClient()}>
              <FontAwesomeIcon icon={solid('floppy-disk')} />
            </Button>
          </InputGroup>
        </td>
        <td>
          <Button variant="danger" onClick={() => deleteClient()}>
            <FontAwesomeIcon icon={solid('trash-can')} />
          </Button>
        </td>
      </tr>
    );
  }
}

ClientRow.propTypes = {
  client: PropTypes.shape({
    name: PropTypes.string.isRequired,
    ip: PropTypes.string.isRequired,
    mac: PropTypes.string.isRequired,
  }),
  refresh: PropTypes.func.isRequired,
};

export default ClientRow;
