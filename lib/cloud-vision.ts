export const GOOGLE_CLIENT_EMAIL = process.env.GOOGLE_CLIENT_EMAIL as string;
export const GOOGLE_PRIVATE_KEY = process.env.GOOGLE_PRIVATE_KEY as string;

import {v1} from '@google-cloud/vision';

export async function textDetection(imageBase64: string) {
  let client = new v1.ImageAnnotatorClient({
    credentials: {
      client_email: GOOGLE_CLIENT_EMAIL,
      private_key: JSON.parse(`"${GOOGLE_PRIVATE_KEY}"`)
    }
  });

  let [res] = await client.textDetection({image: {content: imageBase64}});

  return res?.fullTextAnnotation?.text;
}

