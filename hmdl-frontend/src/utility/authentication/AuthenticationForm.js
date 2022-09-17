import React, { useState } from 'react';
import Alert from 'react-bootstrap/Alert';
import Card from 'react-bootstrap/Card';
import Container from 'react-bootstrap/Container';
import Form from 'react-bootstrap/Form';
import Stack from 'react-bootstrap/Stack';

import RegisterButton from './RegisterButton';
import LoginButton from './LoginButton';

export function AuthenticationForm() {
  const [nickname, setNickname] = useState("");

  if (!window.PublicKeyCredential) {
    return (
      <Container>
        <Alert variant='danger'>
          <h1>Your browser is unsupported</h1>
          <p>
            You must use a device that has webauthn support to use HMDL. <br />
            <a href="https://webauthn.me/browser-support">Test your browser here.</a>
          </p>
        </Alert>
      </Container>
    );
  }

  return (
    <Container className="authenticationBox">
      <Stack gap={2} className="col-md-5 mx-auto">
        <Card>
          <Card.Body>
            <Card.Title>Authentication Required</Card.Title>
            <Form>
              <Form.Group className="mb-3" controlId="formNickname">
                <Form.Label>Nickname</Form.Label>
                <Form.Control
                  autoFocus
                  type="text"
                  placeholder="Enter Nickname"
                  name="nickname"
                  onChange={(event) => setNickname(event.target.value)} />
              </Form.Group>
              <Form.Group>
                <RegisterButton nickname={nickname} />
              </Form.Group>
              <br />
              <Form.Group>
                <LoginButton nickname={nickname} />
              </Form.Group>
            </Form>
          </Card.Body>
        </Card>
      </Stack>
    </Container>
  );
}

export default AuthenticationForm;