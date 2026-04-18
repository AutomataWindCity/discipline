export const HTTP_OK_CODE = 200;
export const HTTP_OK_TEXT = "OK";

export const HTTP_NOT_FOUND_CODE = 404;
export const HTTP_NOT_FOUND_TEXT = "Not Found";

export const HTTP_BAD_REQUEST_CODE = 400;
export const HTTP_BAD_REQUEST_TEXT = "Bad Request";

export const HTTP_INTERNAL_SERVER_ERROR_CODE = 500;
export const HTTP_INTERNAL_SERVER_ERROR_TEXT = "Internal Server Error";

export const createOkHtmlResponse = (message: string) => {
  return new Response(message, {
    status: HTTP_OK_CODE,
    statusText: HTTP_OK_TEXT,
    headers: {
      "Content-Type": "text/html",
    }
  });
};

export const createOkResponse = (message: string) => {
  return new Response(message, {
    status: HTTP_OK_CODE,
    statusText: HTTP_OK_TEXT,
  });
};

export const createNotFoundResponse = (message: string) => {
  return new Response(message, {
    status: HTTP_NOT_FOUND_CODE,
    statusText: HTTP_NOT_FOUND_TEXT,
  });
};

export const createBadRequestResponse = (message: string) => {
  return new Response(message, {
    status: HTTP_BAD_REQUEST_CODE,
    statusText: HTTP_BAD_REQUEST_TEXT,
  });
};

export const createInternalServerErrorResponse = (message: string) => {
  return new Response(message, {
    status: HTTP_INTERNAL_SERVER_ERROR_CODE,
    statusText: HTTP_INTERNAL_SERVER_ERROR_TEXT,
  });
};
