import { createTypedHooks } from 'easy-peasy'

import { storeModel } from './'

const typedHooks = createTypedHooks<typeof storeModel>()

export const useStoreActions = typedHooks.useStoreActions
export const useStoreDispatch = typedHooks.useStoreDispatch
export const useStoreState = typedHooks.useStoreState
