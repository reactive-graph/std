import * as React from 'react'
import * as ReactDOM from 'react-dom'
import {Voyager} from '@guigiani/graphql-voyager'
import introspectionProvider from './IntrospectionProvider'

import '@guigiani/graphql-voyager/dist/voyager.css'
import '../assets/css/graphql-schema-visualization.css'

const urlParams = new URLSearchParams(window.location.search)
const rootType = urlParams.has('rootType') ? urlParams.get('rootType') : 'Subscription'
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

const workerURI = "../voyager.worker.js"

ReactDOM.render(
  <Voyager introspection={introspectionProvider} displayOptions={displayOptions} workerURI={workerURI} hideDocs={hideDocs} hideSettings={hideSettings} />,
  document.getElementById('voyager'),
)
