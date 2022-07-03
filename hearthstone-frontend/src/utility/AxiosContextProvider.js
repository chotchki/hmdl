import { createContext, useEffect, useRef } from "react";

const AxiosContext = createContext(null);
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
    }, []);

    return (
        <AxiosContext.Provider value={instanceRef.current}>
            {children}
        </AxiosContext.Provider>
    );
};

export default { AxiosContext, AxiosInstanceProvider };