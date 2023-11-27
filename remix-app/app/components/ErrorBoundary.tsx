import { Alert, AlertDescription, AlertIcon, AlertTitle, Box, Text } from "@chakra-ui/react";
import {
    isRouteErrorResponse,
    useRouteError,
} from "@remix-run/react";

export default function ErrorBoundary() {
    const error = useRouteError();

    if (isRouteErrorResponse(error)) {
        return (
            <Alert status="error">
                <AlertIcon />
                <AlertTitle>
                    {error.status} {error.statusText}
                </AlertTitle>
                <AlertDescription>{error.data}</AlertDescription>
            </Alert>
        );
    } else if (error instanceof Error) {
        return (
            <Alert status="error">
                <AlertIcon />
                <Box>
                    <AlertTitle>Error</AlertTitle>
                    <AlertDescription>{error.message}</AlertDescription>
                    <Text>The stack trace is:</Text>
                    <pre>{error.stack}</pre>
                </Box>
            </Alert>
        );
    } else {
        return (
            <Alert status="error">
                <AlertIcon />
                <AlertTitle>Unknown Error</AlertTitle>
            </Alert>
        );
    }
}
