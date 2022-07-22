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

export function DomainRow(props) {
  const [{ data, error, loading }] = useAxios({
    url: '/api/domain-groups',
    method: 'GET',
  });
  const [domainName, setDomainName] = useState(props.domain.name);
  const [groupName, setGroupName] = useState('');

  const [{ }, executePut] = useAxios(
    {
      url: '/api/domains/' + props.domain.name,
      method: 'PUT',
    },
    { manual: true },
  );

  const updateDomain = (event) => {
    executePut({
      data: {
        domain: {
          name: domainName,
          last_seen: props.domain.last_seen,
          last_client: props.domain.last_client,
        },
        group_name: groupName,
      },
    }).then(() => {
      props.refresh();
    });
  };

  const [{ }, executeDel] = useAxios(
    {
      url: '/api/domains/' + props.domain.name,
      method: 'DELETE',
    },
    { manual: true },
  );

  const deleteDomain = (event) => {
    executeDel().then(() => {
      props.refresh();
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
          <Form.Control
            type="text"
            defaultValue={domainName}
            onChange={(event) => setDomainName(event.target.value)} />
        </td>
        <td><Timestamp lastSeen={props.domain.last_seen} /></td>
        <td>{props.domain.last_client}</td>
        <td>
          <InputGroup>
            <Form.Select onChange={(event) => setGroupName(event.target.value)}>
              <option>Assign Group</option>
              {data.map((group) => (
                <option key={group}>{group}</option>
              ))}
            </Form.Select>
            <Button variant="primary" onClick={() => updateDomain()}>
              <FontAwesomeIcon icon={solid('floppy-disk')} />
            </Button>
          </InputGroup>
        </td>
        <td>
          <Button variant="danger" onClick={() => deleteDomain()}>
            <FontAwesomeIcon icon={solid('trash-can')} />
          </Button>
        </td>
      </tr >
    );
  }
}

DomainRow.propTypes = {
  domain: PropTypes.shape({
    name: PropTypes.string.isRequired,
    last_client: PropTypes.string.isRequired,
    last_seen: PropTypes.string.isRequired,
  }),
  refresh: PropTypes.func.isRequired,
};

export default DomainRow;
