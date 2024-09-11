import { env } from '$env/dynamic/private';

export const API_BASE_URL = env.FLUX_ATLAS_API_ADDRESS || 'http://localhost:8000/api/';