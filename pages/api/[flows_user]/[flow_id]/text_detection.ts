import { NextRequest, NextResponse } from 'next/server';
import { redis } from '@/lib/upstash';
import { textDetection } from '@/lib/cloud-vision';

export default async (req: any, res: any, next: any) => {
  const {flows_user: flowsUser, flow_id: flowId} = req.query;
  
  if (!flowsUser || !flowId) {
    return new NextResponse('Bad request', {status: 400});
  }
  
  try {

    const imageBase64 = req.body;
    let text = await textDetection(imageBase64);

    // Record the usage count
    {
      let usageKey = `cloud-vision:${flowsUser}/${flowId}:text-detection`;
      let usage: {count: number} | null = await redis.get(usageKey);
      if (!usage || !usage.count) {
        usage = {count: 1};
      } else {
        usage.count++;
      }
      await redis.set(usageKey, usage);
    }

    return res.status(200).end(text);
  } catch(e: any) {
    return res.status(500).end(e.toString());
  }
};

