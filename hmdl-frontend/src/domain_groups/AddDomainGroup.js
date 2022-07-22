import React, { useState } from 'react';
import useAxios from 'axios-hooks';

import Alert from 'react-bootstrap/Alert';
import Button from 'react-bootstrap/Button';
import Card from 'react-bootstrap/Card';
import Form from 'react-bootstrap/Form';
import Spinner from 'react-bootstrap/Spinner';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { solid } from '@fortawesome/fontawesome-svg-core/import.macro';

export function AddGroup(props) {
  const [groupName, setGroupName] = useState(null);
  const [{ loading, error }, executePost] = useAxios(
    {
      method: 'POST',
    },
    { manual: true },
  );

  const submitGroup = (event) => {
    executePost({
      url: '/api/domain-groups/' + groupName,
      data: 'Foo',
    }).then(() => {
      props.refresh();
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
      <Card>
        <Card.Body>
          <Form onSubmit={submitGroup}>
            <Form.Group className="mb-3" controlId="name">
              <Form.Label>Group Name</Form.Label>
              <Form.Control
                type="text"
                placeholder="Enter group name"
                onChange={(event) => setGroupName(event.target.value)} />
            </Form.Group>
            <Button variant="primary" type="submit">
              Add <FontAwesomeIcon icon={solid('plus')} />
            </Button>
          </Form>
        </Card.Body>
      </Card>
    );
  }
}

AddGroup.propTypes = {
  refresh: PropTypes.func.isRequired,
};

export default AddGroup;
