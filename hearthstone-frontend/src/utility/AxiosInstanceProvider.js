import { useEffect, useRef } from "react";
import axios from "axios";
import AxiosContext from "./AxiosContext.js";

const AxiosInstanceProvider = ({
    config = {},
    requestInterceptors = [],
    responseInterceptors = [],
    children,
}) => {
    const instanceRef = useRef(axios.create(config));

    useEffect(() => {
        requestInterceptors.forEach((interceptor) => {
            instanceRef.current.interceptors.request.use(
                interceptor
            );
        });
        responseInterceptors.forEach((interceptor) => {
            instanceRef.current.interceptors.response.use(
                interceptor
            );
        });
    }, [requestInterceptors, responseInterceptors]);

    return (
        <AxiosContext.Provider value={instanceRef.current}>
            {children}
        </AxiosContext.Provider>
    );
};

export default AxiosInstanceProvider;