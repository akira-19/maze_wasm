import { Application } from 'https://deno.land/x/oak/mod.ts';

const app = new Application();

app.use(async (context, next) => {
  try {
    await context.send({
      root: `${Deno.cwd()}/www/public`,
      index: 'index.html',
    });
  } catch {
    await next();
  }
});

await app.listen({ port: 8000 });
