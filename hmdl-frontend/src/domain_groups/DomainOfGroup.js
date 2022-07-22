import React from 'react';
import useAxios from 'axios-hooks';

import Button from 'react-bootstrap/Button';
import ListGroup from 'react-bootstrap/ListGroup';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { solid } from '@fortawesome/fontawesome-svg-core/import.macro';

export function DomainOfGroup(props) {
  const [{ }, executeDel] = useAxios(
    {
      url: '/api/domains/' + props.domain + '/group',
      method: 'DELETE',
    },
    { manual: true },
  );

  const deleteDomainGroup = (event) => {
    executeDel().then(() => {
      props.refresh();
    });
  };

  return (
    <ListGroup.Item>
      <span>{props.domain}</span>
      <Button variant="danger" onClick={() => deleteDomainGroup()}>
        <FontAwesomeIcon icon={solid('trash-can')} />
      </Button>
    </ListGroup.Item>
  );
}

DomainOfGroup.propTypes = {
  domain: PropTypes.string.isRequired,
  refresh: PropTypes.func.isRequired,
};

export default DomainOfGroup;
