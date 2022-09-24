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

export function DomainGroupApplied(props) {
  const { addToastAxiosError, addToastSuccess } = useToast();

  const [{ }, executeDel] = useAxios(
    {
      url: '/api/groups-applied',
      method: 'PUT',
    },
    { manual: true },
  );

  const deleteDomainGroup = (event) => {
    executeDel({
      data: {
        client_group: props.clientGroup,
        domain_group: props.domainGroup,
      },
    }).then(() => {
      addToastSuccess('Domain group removed successfully');
      props.refresh();
    }).catch((e) => {
      addToastAxiosError(e, 'Unable to remove group.');
    });
  };

  return (
    <ListGroup.Item>
      <Container fluid>
        <Stack direction="horizontal" gap={3}>
          <span className="me-auto">
            {props.domainGroup}
          </span>
          <Button variant="danger" onClick={() => deleteDomainGroup()}>
            <FontAwesomeIcon icon={solid('trash-can')} />
          </Button>
        </Stack>
      </Container>
    </ListGroup.Item >
  );
}

DomainGroupApplied.propTypes = {
  clientGroup: PropTypes.string.isRequired,
  domainGroup: PropTypes.string.isRequired,
  refresh: PropTypes.func.isRequired,
};

export default DomainGroupApplied;
