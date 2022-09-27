import React, { useState } from 'react';
import useAxios from 'axios-hooks';
import { useInterval } from 'usehooks-ts';
import { Navigate } from 'react-router-dom';

import Alert from 'react-bootstrap/Alert';
import Container from 'react-bootstrap/Container';
import Spinner from 'react-bootstrap/Spinner';


export function PostSetup() {
  const [{ data, error, loading }, executeIsSetup] = useAxios({
    url: '/api/is-setup',
    method: 'GET',
  }, { manual: true });

  const [isLoading, setLoading] = useState(true);
  const [isSetupError, setSetupError] = useState(false);
  const [count, setCount] = useState(0);


  // TODO Try to reduce the initial load time
  useInterval(
    () => {
      if (count > 3000) {
        setSetupError(true);
      } else if (!loading && !error && data && data.status === 'Setup') {
        setLoading(false);
      } else if (!loading) {
        setCount(count + 1);
        executeIsSetup();
      }
    },
    // Run until we have success or failure
    isLoading ? 500 : null,
  );

  if (isSetupError) {
    return (
      <Container>
        <Alert variant='danger'>
          <h1>The server certificate has not generated in a reasonable time!</h1>
          <p>Go yell at the admin.</p>
        </Alert>
      </Container>
    );
  }

  if (data && data.status === 'Not Setup') {
    return (
      <Navigate to="/pre-setup" replace={false} />
    );
  }

  if (data && data.status === 'In Progress') {
    return (
      <Container>
        <Alert variant='info'>
          <h1>Working on getting HMDL its certificate</h1>
          <p>Please wait...</p>
          <Spinner animation="border" role="status">
            <span className="visually-hidden">Loading...</span>
          </Spinner>
        </Alert>
      </Container>
    );
  }

  if (data && data.status === 'Setup') {
    if (data.domain !== window.location.hostname || window.location.protocol !== 'https:') {
      // Part of setup is switching to https so this accomplishes that
      window.location.href = 'https://' + data.domain;
    } else {
      return (
        <Navigate to="/authentication" replace={false} />
      );
    }
  }

  return (
    <Container>
      <Alert variant='info'>
        <h1>Checking Server Setup Status</h1>
        <p>Please wait...</p>
        <Spinner animation="border" role="status">
          <span className="visually-hidden">Loading...</span>
        </Spinner>
      </Alert>
    </Container>
  );
}

export default PostSetup;
