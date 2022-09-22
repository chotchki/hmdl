import React, { useState } from 'react';
import useAxios from 'axios-hooks';
import { Navigate, useNavigate } from 'react-router-dom';

import Alert from 'react-bootstrap/Alert';
import Button from 'react-bootstrap/Button';
import Container from 'react-bootstrap/Container';
import Form from 'react-bootstrap/Form';
import Spinner from 'react-bootstrap/Spinner';

import { useToast } from '../toaster/ToastProvider';

export function PreSetup() {
  const { addToastAxiosError, addToastSuccess } = useToast();
  const [{ data, error, loading }] = useAxios({
    url: '/api/is-setup',
    method: 'GET',
  });

  const [{ }, executeSetup] = useAxios({
    url: '/api/setup',
    method: 'POST',
  }, { manual: true });

  const navigate = useNavigate();
  const [domainName, setDomainName] = useState('');
  const [cloudflareToken, setCloudflareToken] = useState('');
  const [acmeEmail, setAcmeEmail] = useState('');

  const submitSetup = (event) => {
    executeSetup({
      data: {
        application_domain: domainName,
        cloudflare_api_token: cloudflareToken,
        acme_email: acmeEmail,
      },
    }).then(() => {
      addToastSuccess('Application was successfully setup. Pending certificate issuance.');
      navigate('/post-setup');
    }).catch((e) => {
      addToastAxiosError(e, 'Had an error setting up the application.');
    });
  };

  if (loading) {
    return (
      <Container>
        <Alert variant='info'>
          <h1>Checking if HMDL is setup</h1>
          <p>Please wait...</p>
          <Spinner animation="border" role="status">
            <span className="visually-hidden">Loading...</span>
          </Spinner>
        </Alert>
      </Container>
    );
  }

  if (error) {
    return (
      <Container>
        <Alert variant='danger'>
          <h1>Had an issue checking setup, please look at the logs!</h1>
        </Alert>
      </Container>
    );
  }

  if (data.status !== 'Not Setup') {
    return (
      <Navigate to="/post-setup" replace={false} />
    );
  }

  return (
    <>
      <Container>
        <h1>Let &apos;s Setup HMDL!</h1>
        <Form onSubmit={submitSetup}>
          <Form.Group className="mb-3" controlId="domainName">
            <Form.Label>Enter a domain name to have HMDL use.</Form.Label>
            <Form.Control
              type="text"
              placeholder="Domain name"
              onChange={(event) => setDomainName(event.target.value)} />
          </Form.Group>
          <Form.Group className="mb-3" controlId="domainName">
            <Form.Label>Enter the token that can update the dns record above</Form.Label>
            <Form.Control
              type="text"
              placeholder="CloudFlare API Token"
              onChange={(event) => setCloudflareToken(event.target.value)} />
          </Form.Group>
          <Form.Group className="mb-3" controlId="acmePrivateKey">
            <Form.Label>An email address is needed to get certificate notifications</Form.Label>
            <Form.Control
              type="email"
              placeholder="Enter ACME Email to Generate the Private Key"
              onChange={(event) => setAcmeEmail(event.target.value)} />
          </Form.Group>
          <Button variant="primary" type="submit">
            Start Setup
          </Button>
        </Form>
      </Container>
    </>
  );
}

export default PreSetup;
