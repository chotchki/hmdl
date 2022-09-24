import React from 'react';
import useAxios from 'axios-hooks';
import PropTypes from 'prop-types';

import Button from 'react-bootstrap/Button';
import Container from 'react-bootstrap/Container';
import ListGroup from 'react-bootstrap/ListGroup';
import Stack from 'react-bootstrap/Stack';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { solid } from '@fortawesome/fontawesome-svg-core/import.macro';

import { useToast } from '../utility/toaster/ToastProvider';

export function ClientOfGroup(props) {
  const { addToastAxiosError, addToastSuccess } = useToast();

  const [{ }, executeDel] = useAxios(
    {
      url: '/api/clients/' + props.client.name + '/group',
      method: 'DELETE',
    },
    { manual: true },
  );

  const deleteClientGroup = (event) => {
    executeDel().then(() => {
      addToastSuccess('Client removed successfully');
      props.refresh();
    }).catch((e) => {
      addToastAxiosError(e, 'Unable to delete client.');
    });
  };

  return (
    <ListGroup.Item>
      <Container fluid>
        <Stack direction="horizontal" gap={3}>
          <span className="me-auto">
            {props.client.name} - {props.client.ip} - {props.client.mac}
          </span>
          <Button variant="danger" onClick={() => deleteClientGroup()}>
            <FontAwesomeIcon icon={solid('trash-can')} />
          </Button>
        </Stack>
      </Container>
    </ListGroup.Item >
  );
}

ClientOfGroup.propTypes = {
  client: PropTypes.shape({
    name: PropTypes.string.isRequired,
    ip: PropTypes.string.isRequired,
    mac: PropTypes.string.isRequired,
  }),
  refresh: PropTypes.func.isRequired,
};

export default ClientOfGroup;
