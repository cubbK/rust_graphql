// Fake federation, not even a federation but just something to show that it's interacting with the other graphs
import Koa from "koa";
import { request, gql } from "graphql-request";

const queryGraph1 = gql`
  {
    members {
      name
    }
  }
`;

const queryGraph2 = gql`
  {
    members {
      name
    }
  }
`;

const app = new Koa();

app.use(async (ctx) => {
  const result1 = await request("http://localhost:8081/graphql", queryGraph1);
  const result2 = await request("http://localhost:8082/graphql", queryGraph2);

  ctx.body = `
    It's working!
    ${JSON.stringify(result1, null, 2)}
    ${JSON.stringify(result2, null, 2)}
  `;
});

console.log("Server running on port 3000");
app.listen(3000);
