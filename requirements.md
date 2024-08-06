1. Solução funcional e com objetivo explícito. Deve dividir, de maneira adequada,
componentes importantes da aplicação, front e back, criando seus próprios pacotes e
subdiretórios. Todos os serviços de sua aplicação devem seguir a arquitetura MVC.
Para cada modelo operacional, implementar o respectivo controller - rest controller -
que deve receber as requisições e respondê-las. NÃO implementar lógica e regras de
negócio nos controllers. Para isso, você deve usar os services, desacoplando sua
aplicação das classes repositórios ou de regras de negócio. (1,00 pontos)

R: A aplicação foi dividida em 4 partes: client, gateway, service discovery e services. O client é a aplicação frontend, o gateway é a aplicação que faz a comunicação entre o frontend e os serviços, o service discovery é a aplicação que faz a descoberta dos serviços e os services são as aplicações que contém a lógica de negócio. Cada serviço segue a arquitetura MVC, com um controller, um service e um repository. A lógica de negócio foi implementada nos services. 

2. O sistema deve fazer extenso uso do padrão arquitetural de microsserviços. Você deve
escalar sua aplicação em, pelo menos, 4 microsserviços principais - núcleo das
funções (não contabiliza api gateway, serviço de descoberta e serviço de
configuração). Os microsserviços devem se registrar em um serviço de descoberta.
Além disso, deve existir API gateway sendo ponto único de comunicação do cliente
com os outros serviços da sua aplicação. (3,00 pontos)

R: A aplicação foi dividida em 4 microsserviços principais: auth, class, exam e question. Cada microsserviço se registra no serviço de descoberta e a comunicação entre os microsserviços é feita através do gateway.

Auth: microsserviço responsável pela autenticação dos usuários.
Class: microsserviço responsável pela logica por trás das turmas.
Exam: microsserviço responsável pela logica por trás das provas e sua execução.
Question: microsserviço responsável pela logica por trás das questões e criação de provas.


3. Todos os serviços devem obter suas configurações a partir de um servidor de
configuração integrado a um repositório no Git. Informações relacionadas a bancos,
acessos, strings sensíveis devem ficar em um arquivo de perfil de acordo com uso e
serem carregadas pelo serviço de configuração. (1,00 pontos)

R: Nesse ponto, a minha ideia era implementar ci/cd, salvando as configurações no git e fazendo o deploy automático. Porém, não consegui implementar dentro do tempo. Então, as variáveis de ambiente estão sendo passadas diretamente no docker-compose.yml para cada serviço.

Uma ideia, também, era criar um repositório no git com todas as envs para cada serviço. No ci/cd, essas envs seriam recuperadas e passadas para cada serviço.


4. Segurança: você deve implementar camada de segurança no acesso aos recursos dos
seus microsserviços. Autenticar os usuários que terão acesso. Deve existir pelo menos
4 papéis de usuários no seu sistema que terão permissões de acesso personalizadas.
Você deve utilizar tokens para manter seus usuários autenticados - recomendo o JWT.
O token deve ter um prazo de expiração, sendo necessário uma nova autenticação com
login e senha. Pelo menos um conjunto de requisições de uma entidade deve ser
habilitada só a usuários com papel de administrador ou a mais de um papel. (2,00
pontos)

R: A autenticação é feita através de JWT. O token tem um prazo de expiração de 1 hora. Existem 4 papéis de usuários: admin, teacher, student e monitor. Apenas o admin tem acesso a todas as rotas. 
O professor tem acesso a criação de questões, provas, turmas e provas. O aluno tem consegue se inscrever nas turmas, realizar suas provas e visualizar seus resultados. A ideia do monitor, era ser uma entidade com acesso aos resultados das provas dos alunos, mas não consegui implementar a tempo a sua visão no frontend.

5. Aplicação cliente: Sua solução deve conter uma aplicação cliente - frontend - que
entregará às funcionalidades ao usuário. Defina um framework de sua preferência
(angular, react, etc) ou use javascript puro com html e css. Implemente componentes
visuais que propiciem experiência de usuário adequada. Personalize os componentes.
Sua aplicação cliente deve se integrar ao seu backend - arquitetura de microsserviços -
através da api gateway (3,00 pontos).

R: O frontend foi feito em Nextjs (React) e tailwind. A aplicação se integra com o backend através do gateway, que inclusive é o único que está exposto no docker-compose. 