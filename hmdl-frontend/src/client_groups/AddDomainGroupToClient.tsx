import { useState } from 'react';
import useAxios from 'axios-hooks';
import Alert from 'react-bootstrap/Alert';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import InputGroup from 'react-bootstrap/InputGroup';
import Spinner from 'react-bootstrap/Spinner';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { solid } from '@fortawesome/fontawesome-svg-core/import.macro';
import { useToast } from '../utility/toaster/ToastProvider';

type AddDomainGroupToClientProps = {
  client_group: string,
  refresh: () => void,
};

const AddDomainGroupToClient = ({ client_group, refresh }: AddDomainGroupToClientProps): JSX.Element => {
  const { addToastAxiosError, addToastSuccess } = useToast();
  const [{ data, error, loading }] = useAxios({ url: '/api/domain-groups', method: 'GET' });

  const [domainGroup, setDomainGroup] = useState<string | null>(null);

  const [{ }, executePost] = useAxios(
    {
      url: '/api/groups-applied',
      method: 'POST',
    },
    { manual: true },
  );

  const assignGroup = () => {
    executePost({
      data: {
        client_group: client_group,
        domain_group: domainGroup,
      },
    }).then(() => {
      addToastSuccess(domainGroup + ' was successfully assigned.');
      refresh();
    }).catch((e) => {
      addToastAxiosError(e, 'Unable to assign group.');
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
      <Form>
        <InputGroup>
          <Form.Select onChange={(event) => setDomainGroup(event.target.value)}>
            <option>Assign Domain Group</option>
            {data.map((group: string) => (
              <option key={group}>{group}</option>
            ))}
          </Form.Select>
          <Button variant="primary" onClick={() => assignGroup()}>
            <FontAwesomeIcon icon={solid('plus')} />
          </Button>
        </InputGroup>
      </Form>
    );
  }
}

export default AddDomainGroupToClient;
