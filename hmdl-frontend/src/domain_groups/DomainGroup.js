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

import DomainOfGroup from './DomainOfGroup';
import { useToast } from '../utility/toaster/ToastProvider';

export function DomainGroup() {
  const { addToastAxiosError, addToastSuccess } = useToast();
  const navigate = useNavigate();
  const { group } = useParams();

  const [{ data, error, loading }, executeGet] = useAxios(
    {
      url: '/api/domain-groups/' + group,
      method: 'GET',
    },
  );

  const [newGroupName, setNewGroupName] = useState('');
  const [modelStatus] = useState('');

  const [{ }, executePut] = useAxios(
    {
      url: '/api/domain-groups/' + group,
      method: 'PUT',
    },
    { manual: true },
  );

  const updateGroup = (event) => {
    executePut({
      data: {
        name: newGroupName,
        model_status: modelStatus,
      },
    }).then(() => {
      addToastSuccess('Group ' + group + ' renamed to ' + newGroupName + ' successfully');
      navigate('/domain-groups/' + newGroupName);
    }).catch((e) => {
      addToastAxiosError(e, 'Unable to rename group.');
    });
  };

  const [{ }, executeDel] = useAxios(
    {
      url: '/api/domain-groups/' + group,
      method: 'DELETE',
    },
    { manual: true },
  );

  const deleteGroup = () => {
    executeDel().then(() => {
      addToastSuccess('Group ' + group + ' deleted successfully');
      navigate('/domain-groups');
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
          <LinkContainer to="/domain-groups">
            <Breadcrumb.Item>Back to Domain Groups</Breadcrumb.Item>
          </LinkContainer>
          <Breadcrumb.Item active>
            {group}
          </Breadcrumb.Item>
        </Breadcrumb>
        <h4>Edit Group</h4>
        <Form>
          <Form.Group className="mb-3" controlId="group">
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
        <h4>Associated Domains</h4>
        <ListGroup>
          {data.domains.length > 0 ? data.domains.map((domain) => (
            <DomainOfGroup
              key={domain}
              domain={domain}
              refresh={executeGet} />
          )) : <ListGroup.Item>No domains</ListGroup.Item>
          }
        </ListGroup>
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

export default DomainGroup;
