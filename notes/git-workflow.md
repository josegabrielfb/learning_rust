# Git Workflow — Branches, Pull Requests e GitHub Projects

---

## Fluxo por Tarefa (Branch → PR → Merge)

### 1. Antes de começar uma tarefa

```powershell
git checkout main          # garante que está na main
git pull origin main       # sincroniza com o remoto
git checkout -b tarefa4    # cria e entra na nova branch
```

### 2. Durante o desenvolvimento

```powershell
git add .                          # adiciona todos os arquivos
git commit -m "tarefa4: resolve exercício de strings"
git push origin tarefa4            # envia para o GitHub (CI roda aqui)
```

> Faça commits pequenos e frequentes. Mensagem no formato:
> `tarefa4: descrição curta do que foi feito`

### 3. Abrir um Pull Request

1. Acesse o repositório no GitHub
2. Aparecerá um banner "Compare & pull request" — clique nele
3. Preencha:
   - **Título:** `Tarefa 4 — <nome do exercício>`
   - **Descrição:** o que foi implementado, dificuldades, etc.
4. Clique em **"Create pull request"**
5. O CI roda automaticamente — aguarde o ✅

### 4. Merge na main

Após o CI passar (✅):
1. Clique em **"Merge pull request"** → **"Confirm merge"**
2. Delete a branch (o GitHub oferece o botão automaticamente)
3. Localmente, limpe:

```powershell
git checkout main
git pull origin main
git branch -d tarefa4    # deleta a branch local
```

---

## GitHub Issues como Kanban

Sim — o GitHub tem um sistema de **Projects** com board estilo Kanban.

### Criar um Issue (card de tarefa)

1. Vá em **Issues** → **New issue**
2. Preencha título e descrição
3. Adicione **labels** como `tarefa`, `em progresso`, `concluído`

### Criar um board Kanban

1. Vá em **Projects** → **New project**
2. Escolha o template **"Board"**
3. Colunas padrão: `Todo` | `In Progress` | `Done`
4. Crie cards ou vincule Issues existentes

### Vincular Issue a um PR

No corpo do PR, escreva:
```
Closes #5
```
Quando o PR for mergeado, o Issue #5 fecha automaticamente e o card vai para `Done`.

---

## Notificações por Email do CI

### Opção 1 — Notificação automática do GitHub (recomendado)

O GitHub **já envia email** quando um workflow falha, sem nenhuma configuração extra.

Garanta que está ativado em:
**GitHub → Settings → Notifications → Actions** → marque "Email"

> Você recebe email só quando falha. Para sucesso, o GitHub não notifica por padrão (o que é o comportamento correto — você só quer saber quando algo quebra).

### Opção 2 — Email customizado no workflow

Se quiser controle total (ex: email em todo push, com resumo), adicione uma etapa no `ci.yml`:

```yaml
- name: Notificar por email
  if: failure()   # só envia se algo falhar
  uses: dawidd6/action-send-mail@v3
  with:
    server_address: smtp.gmail.com
    server_port: 465
    username: ${{ secrets.EMAIL_USER }}
    password: ${{ secrets.EMAIL_PASSWORD }}
    subject: "❌ CI falhou — ${{ github.repository }}"
    body: "Workflow falhou no commit ${{ github.sha }}. Veja: ${{ github.server_url }}/${{ github.repository }}/actions"
    to: seu@email.com
    from: seu@email.com
```

> As credenciais ficam em **GitHub → Settings → Secrets and variables → Actions**, nunca no código.

---

## Resumo do Fluxo Completo

```
Issue criado no board
       ↓
git checkout -b tarefaN
       ↓
commits + push → CI roda
       ↓
Abrir PR vinculado ao Issue
       ↓
CI passa ✅ → Merge na main
       ↓
Issue fecha automaticamente → card vai para Done
```
