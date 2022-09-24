import React, { useState } from 'react';
import useAxios from 'axios-hooks';
import { useNavigate, useParams } from 'react-router-dom';

import Alert from 'react-bootstrap/Alert';
import Breadcrumb from 'react-bootstrap/Breadcrumb';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import InputGroup from 'react-bootstrap/InputGroup';
import { LinkContainer } from 'react-router-bootstrap';
import ListGroup from 'react-bootstrap/ListGroup';
import Spinner from 'react-bootstrap/Spinner';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { solid } from '@fortawesome/fontawesome-svg-core/import.macro';

import AddDomainGroupToClient from './AddDomainGroupToClient';
import ClientOfGroup from './ClientOfGroup';
import DomainGroupApplied from './DomainGroupApplied';
import { useToast } from '../utility/toaster/ToastProvider';

export function ClientGroup() {
  const { addToastAxiosError, addToastSuccess } = useToast();
  const navigate = useNavigate();
  const { group } = useParams();

  const [{ data, error, loading }, executeGet] = useAxios(
    {
      url: '/api/client-groups/' + group,
      method: 'GET',
    },
  );

  const [newGroupName, setNewGroupName] = useState('');

  const [{ }, executePut] = useAxios(
    {
      url: '/api/client-groups/' + group,
      method: 'PUT',
    },
    { manual: true },
  );

  const updateGroup = (event) => {
    executePut({
      data: {
        name: newGroupName,
      },
    }).then(() => {
      addToastSuccess('Group ' + group + ' renamed to ' + newGroupName + ' successfully');
      navigate('/client-groups/' + newGroupName);
    }).catch((e) => {
      addToastAxiosError(e, 'Unable to rename group.');
    });
  };

  const [{ }, executeDel] = useAxios(
    {
      url: '/api/client-groups/' + group,
      method: 'DELETE',
    },
    { manual: true },
  );

  const deleteGroup = () => {
    executeDel().then(() => {
      addToastSuccess('Group ' + group + ' deleted successfully');
      navigate('/client-groups');
    }).catch((e) => {
      addToastAxiosError(e, 'Unable to delete group.');
    });
  };

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
      <>
        <Breadcrumb>
          <LinkContainer to="/client-groups">
            <Breadcrumb.Item>Back to Client Groups</Breadcrumb.Item>
          </LinkContainer>
          <Breadcrumb.Item active>
            {group}
          </Breadcrumb.Item>
        </Breadcrumb>
        <h4>Edit Group</h4>
        <Form>
          <Form.Group className="mb-3" controlId="groupName">
            <Form.Label>Group Name</Form.Label>
            <InputGroup>
              <Form.Control
                type="text"
                defaultValue={group}
                onChange={(event) => setNewGroupName(event.target.value)} />
              <Button variant="primary" onClick={() => updateGroup()}>
                <FontAwesomeIcon icon={solid('floppy-disk')} />
              </Button>
            </InputGroup>
          </Form.Group>
        </Form>
        <h4>Associated Clients</h4>
        <ListGroup>
          {data.clients.length > 0 ? data.clients.map((client) => (
            <ClientOfGroup key={client.name} client={client} refresh={executeGet} />
          )) : <ListGroup.Item>No clients</ListGroup.Item>
          }
        </ListGroup>
        <h4>Domain Groups to Block</h4>
        <ListGroup>
          {data.domain_groups.length > 0 ? data.domain_groups.map((domainGroup) => (
            <DomainGroupApplied
              key={domainGroup.domain_group}
              clientGroup={group}
              domainGroup={domainGroup}
              refresh={executeGet} />
          )) : <ListGroup.Item>No domain groups</ListGroup.Item>}
        </ListGroup>
        <AddDomainGroupToClient client_group={group} refresh={executeGet} />
        <h4>Danger!</h4>
        <Form>
          <Button variant="danger" onClick={() => deleteGroup()}>
            Delete Group <FontAwesomeIcon icon={solid('trash-can')} />
          </Button>
        </Form>
      </>
    );
  }
}

export default ClientGroup;

