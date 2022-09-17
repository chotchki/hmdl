import React, { useEffect, useState } from 'react';
import PropTypes from 'prop-types';
import { useNavigate } from "react-router-dom";
import useAxios from 'axios-hooks';
import {
  create,
  get,
  parseCreationOptionsFromJSON,
} from "@github/webauthn-json/browser-ponyfill";

import Button from 'react-bootstrap/Button';

import { useToast } from '../toaster/ToastProvider';
import { useAuthentication } from './AuthenticationProvider';

export function RegisterButton(props) {
  const { setRole } = useAuthentication();
  const navigate = useNavigate();
  const { addToastAxiosError, addToastSuccess } = useToast();

  const [{ }, startRegister] = useAxios(
    {
      url: '/api/auth/register_start',
      method: 'POST',
    },
    { manual: true },
  );

  const [{ }, finishRegister] = useAxios(
    {
      url: '/api/auth/register_finish',
      method: 'POST',
    },
    { manual: true },
  );

  const registerStart = (event) => {
    startRegister({
      data: { "username": props.nickname },
    }).then((data) => {
      setRegChallenge(parseCreationOptionsFromJSON(data.data));
    }).catch((e) => {
      addToastAxiosError(e, "Unable to create credential.");
    });
  };

  //From https://devtrium.com/posts/async-functions-useeffect
  //The goal is to set the parameter from setRegChallenge
  const [regChallenge, setRegChallenge] = useState(null);
  const [cred, setCred] = useState(null);
  useEffect(() => {
    let isSubscribed = true;

    // declare the async data fetching function
    const loadRegCredential = async () => {
      // get the data from the api
      if (regChallenge !== null && isSubscribed) {
        const data = await create(regChallenge);
        finishRegister({
          data: { "reg_pub_cred": data }
        }).then((role) => {
          setRole(role);
          navigate('/domains');
        }).catch((e) => {
          addToastAxiosError(e, "Error completing registration.");
        });

        // set state with the result if `isSubscribed` is true
        if (isSubscribed) {
          setCred(data);
        }
      }
    }

    // call the function
    loadRegCredential()
      // make sure to catch any error
      .catch(console.error);

    // cancel any future `setData`
    return () => isSubscribed = false;
  }, [addToastAxiosError, regChallenge, finishRegister, navigate, setRole]);


  return (
    <Button
      variant="primary" size="lg"
      onClick={() => registerStart()}
    >
      Register
    </Button>
  );
};

RegisterButton.propTypes = {
  nickname: PropTypes.string.isRequired,
};

export default RegisterButton;