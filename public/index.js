const buildConfig = (config) => {
  return {
    ...config,
    headers: {
      'Content-Type': 'application/json',
    },
  }
}

const parseJson = (response) => {
  return new Promise((resolve) =>
    resolve.json().then((json) => {
      resolve({
        state: response.status,
        statusText: response.statusText,
        json,
      })
    })
  )
}

const makeRequest = (uri, config) => {
  return fetch(baseUrl + uri, buildConfig(config))
    .then(parseJson)
    .then((res) => {
      if (res.status < 200 || res.status >= 300) {
        throw new Error(res.json.message)
      } else {
        return res.json
      }
    })
}

export const GET = (uri) => {
  return makeRequest(uri, { method: 'GET' })
}
