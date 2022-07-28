import React, { useState } from 'react';
import useAxios from 'axios-hooks';
import { useInterval } from 'usehooks-ts';
import { useNavigate } from 'react-router-dom';

import Alert from 'react-bootstrap/Alert';
import Container from 'react-bootstrap/Container';
import Spinner from 'react-bootstrap/Spinner';

export function Health() {
  const [{ data, error, loading }, executeHealth] = useAxios({
    url: '/api/health',
    method: 'GET',
  }, { manual: true });
  const navigate = useNavigate();

  const [isLoading, setLoading] = useState(true);
  const [isHealthError, setHealthError] = useState(true);
  const [count, setCount] = useState(0);


  // TODO Try to reduce the initial load time
  useInterval(
    () => {
      if (count > 300) {
        setHealthError(true);
      } else if (!loading && !error && data === 'Ok') {
        setLoading(false);
        navigate('/pre-setup');
      } else if (!loading) {
        setCount(count + 1);
        executeHealth();
      }
    },
    // Run until we have success or failure
    isLoading ? 50 : null,
  );

  if (isLoading) {
    return (
      <Container>
        <Alert variant='info'>
          <h1>Backend Server is loading</h1>
          <p>Please wait...</p>
          <Spinner animation="border" role="status">
            <span className="visually-hidden">Loading...</span>
          </Spinner>
        </Alert>
      </Container>
    );
  }

  if (isHealthError) {
    return (
      <Container>
        <Alert variant='danger'>
          <h1>The server is not coming up in a reasonable time!</h1>
          <p>Go yell at the admin.</p>
        </Alert>
      </Container>
    );
  }
}

export default Health;
