import * as React from 'react'
import * as ReactDOMClient from 'react-dom/client'
import { Voyager } from 'graphql-voyager'

const urlParams = new URLSearchParams(window.location.search)
const rootType = urlParams.has('rootType') ? urlParams.get('rootType') : 'Query'
const showLeafFields = urlParams.has('showLeafFields') ? urlParams.get('showLeafFields') === 'true' : true
const skipDeprecated = urlParams.has('skipDeprecated') ? urlParams.get('skipDeprecated') === 'true' : true
const sortByAlphabet = urlParams.has('sortByAlphabet') ? urlParams.get('sortByAlphabet') === 'true' : false
const skipRelay = urlParams.has('skipRelay') ? urlParams.get('skipRelay') === 'true' : true
const hideRoot = urlParams.has('hideRoot') ? urlParams.get('hideRoot') === 'true' : true
const displayOptions = {
  rootType,
  showLeafFields,
  skipDeprecated,
  sortByAlphabet,
  skipRelay,
  hideRoot
}
const hideDocs = urlParams.has('hideDocs') ? urlParams.get('hideDocs') === 'true' : false
const hideSettings = urlParams.has('hideSettings') ? urlParams.get('hideSettings') === 'true' : false

const protocol = urlParams.has('protocol') ? urlParams.get('protocol') : window.location.protocol
const hostname = urlParams.has('hostname') ? urlParams.get('hostname') : window.location.hostname
const port = urlParams.has('port') ? urlParams.get('port') : window.location.port
const endpoint = urlParams.has('endpoint') ? urlParams.get('endpoint') : '/graphql'
const schemaUrl = `${protocol}//${hostname}:${port}${endpoint}`

async function introspectionProvider(query) {
  const response = await fetch(
    schemaUrl,
    {
      method: 'post',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ query }),
      credentials: 'omit'
    }
  )
  return response.json()
}

const reactRoot = ReactDOMClient.createRoot(document.getElementById('voyager'));
reactRoot.render(
  <Voyager
    introspection={introspectionProvider}
    displayOptions={displayOptions}
    hideDocs={hideDocs}
    hideSettings={hideSettings}
  />,
);
