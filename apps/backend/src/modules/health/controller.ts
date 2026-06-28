import { Context } from 'hono';

export class HealthController {
  static async getRoot(c: Context) {
    return c.json({
      success: true,
      message: 'Spore - Online Package Repository Server',
      version: '0.1.0',
      timestamp: new Date().toISOString(),
    });
  }

  static async getHealth(c: Context) {
    return c.json({
      success: true,
      status: 'healthy',
      service: 'spore-space',
      version: '0.1.0',
      timestamp: new Date().toISOString(),
    });
  }
}