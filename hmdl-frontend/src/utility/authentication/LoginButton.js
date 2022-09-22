import React, { useEffect, useState } from 'react';
import PropTypes from 'prop-types';
import { useNavigate } from 'react-router-dom';
import useAxios from 'axios-hooks';
import {
  get,
  parseRequestOptionsFromJSON,
} from '@github/webauthn-json/browser-ponyfill';

import Button from 'react-bootstrap/Button';

import { useToast } from '../toaster/ToastProvider';

export function LoginButton(props) {
  const navigate = useNavigate();
  const { addToastAxiosError } = useToast();

  const [{ }, startLogin] = useAxios(
    {
      url: '/api/auth/login_start',
      method: 'POST',
    },
    { manual: true },
  );

  const [{ }, finishLogin] = useAxios(
    {
      url: '/api/auth/login_finish',
      method: 'POST',
    },
    { manual: true },
  );

  const loginStart = (event) => {
    startLogin({
      data: { 'username': props.nickname },
    }).then((data) => {
      const parse = parseRequestOptionsFromJSON(data.data);
      setAuthChallenge(parse);
    }).catch((e) => {
      addToastAxiosError(e, 'Unable to login.');
    });
  };

  // From https://devtrium.com/posts/async-functions-useeffect
  // The goal is to set the parameter from setChallenge
  const [authChallenge, setAuthChallenge] = useState(null);
  useEffect(() => {
    let isSubscribed = true;

    // declare the async data fetching function
    const loadAuthCredential = async () => {
      // get the data from the api
      if (authChallenge !== null && isSubscribed) {
        const data = await get(authChallenge);
        finishLogin({
          data: { 'pub_cred': data },
        }).then((data) => {
          navigate('/domains');
        }).catch((e) => {
          addToastAxiosError(e, 'Error completing registration.');
        });
      }
    };

    // call the function
    loadAuthCredential()
      // make sure to catch any error
      .catch(console.error);

    // cancel any future `setData`
    return () => isSubscribed = false;
  }, [addToastAxiosError, authChallenge, finishLogin, navigate]);

  return (
    <Button variant="primary" size="lg"
      onClick={() => loginStart()}
    >
      Login
    </Button>
  );
};

LoginButton.propTypes = {
  nickname: PropTypes.string.isRequired,
};

export default LoginButton;
