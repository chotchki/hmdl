import React, { useState } from 'react';
import useAxios from 'axios-hooks';

import Accordion from 'react-bootstrap/Accordion';
import Button from 'react-bootstrap/Button';
import Container from 'react-bootstrap/Container';
import Form from 'react-bootstrap/Form';
import InputGroup from 'react-bootstrap/InputGroup';
import ListGroup from 'react-bootstrap/ListGroup';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { solid } from '@fortawesome/fontawesome-svg-core/import.macro';

import DomainOfGroup from './DomainOfGroup';

export function GroupRow(props) {
  const [groupName, setGroupName] = useState(props.group);
  const [modelStatus] = useState('');

  const [{ }, executePut] = useAxios(
    {
      url: '/api/domain-groups/' + props.group.name,
      method: 'PUT',
    },
    { manual: true },
  );

  const updateGroup = (event) => {
    executePut({
      data: {
        name: groupName,
        model_status: modelStatus,
      },
    }).then(() => {
      props.refresh();
    });
  };

  const [{ }, executeDel] = useAxios(
    {
      url: '/api/domain-groups/' + props.group.name,
      method: 'DELETE',
    },
    { manual: true },
  );

  const deleteGroup = (event) => {
    executeDel().then(() => {
      props.refresh();
    });
  };

  const [{ data: domainGroupDetail }, executeGet] = useAxios(
    {
      url: '/api/domain-groups/' + groupName,
      method: 'GET',
    },
    { manual: true },
  );

  return (
    <Accordion.Item eventKey={props.group.name}>
      <Accordion.Header onClick={() => executeGet()}>{groupName}</Accordion.Header>
      <Accordion.Body>
        <Form>
          <Form.Group className="mb-3" controlId="groupName">
            <Form.Label>Group Name</Form.Label>
            <InputGroup>
              <Form.Control
                type="text"
                defaultValue={groupName}
                onChange={(event) => setGroupName(event.target.value)} />
              <Button variant="primary" onClick={() => updateGroup()}>
                <FontAwesomeIcon icon={solid('floppy-disk')} />
              </Button>
            </InputGroup>
          </Form.Group>
        </Form>
        <h4>Associated Domains</h4>
        <ListGroup>
          {domainGroupDetail ? domainGroupDetail.domains.map((domain) => (
            <DomainOfGroup
              key={domain.name}
              domain={domain.name}
              group={groupName}
              refresh={executeGet} />
          )) : <ListGroup.Item>No domains</ListGroup.Item>
          }
        </ListGroup>
        <Container>
          <Form>
            <Button variant="danger" onClick={() => deleteGroup()}>
              Delete Group <FontAwesomeIcon icon={solid('trash-can')} />
            </Button>
          </Form>
        </Container>
      </Accordion.Body>
    </Accordion.Item >
  );
}

GroupRow.propTypes = {
  group: PropTypes.shape(
    {
      name: PropTypes.string.isRequired,
    },
  ),
  refresh: PropTypes.func.isRequired,
};

export default GroupRow;
