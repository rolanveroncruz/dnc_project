import {Component, DestroyRef, inject, OnInit, signal} from '@angular/core';
import {User, UserService} from '../../../api_services/user-service';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {MatCard, MatCardHeader, MatCardContent,MatCardTitle, MatCardSubtitle} from '@angular/material/card';
import {MatDialog} from '@angular/material/dialog';
import { AddEditUserDialogComponent} from './add-edit-user/add-edit-user';
import {Role, RolesAndPermissionsService} from '../../../api_services/roles-and-permissions-service';

type LoadState = 'loading' | 'loaded' | 'error';
@Component({
  selector: 'app-setup-users',
  imports: [
    MatCard,
    MatCardHeader,
    MatCardContent,
    GenericDataTableComponent,
    MatCardTitle,
    MatCardSubtitle
  ],
  templateUrl: './setup-users.html',
  styleUrl: './setup-users.scss',
})
export class SetupUsers implements OnInit {
  state = signal<LoadState>('loading');
  users = signal<User[] | null>(null);
  errorMsg = signal<string | null>(null);
  roles = signal<Role[] | null>(null);
  roles_state = signal<LoadState>('loading');
  private destroyRef = inject(DestroyRef);

  userColumns: TableColumn[] = [
    {key: 'id', label: 'ID'},
    {key: 'name', label: 'Name'},
    {key: 'email', label: 'Email'},
    {key: 'role', label: 'Role'},
    {key: 'last_modified_by', label: 'Last Modified By'},
    {key: 'last_modified_on', label: 'Last Modified On'},
  ];
  constructor(private userService: UserService, private roles_and_permission_Service: RolesAndPermissionsService) {}

  private dialog = inject(MatDialog);

  ngOnInit(): void {
    this.load_users();
    this.load_roles();
  }

  private load_users() {
    this.state.set('loading');
    this.errorMsg.set(null)
    this.userService.getUsers()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: (res) => {
          this.users.set(res.items);
          this.state.set('loaded');
        },
        error: (err) => {
          console.log("In load(), failed to load users", err);
          this.errorMsg.set("In load(), Failed to load users");
          this.state.set('error');
        }
      })
  }
  load_roles() {
    console.log("In load_roles()");
    this.roles_state.set('loading');
    this.roles_and_permission_Service.getRoles()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: (res) => {
          this.roles.set(processRoles(res.items));
          this.roles_state.set('loaded');
          console.log("In load(), roles:", this.roles());
        },
        error: (err) => {
          console.log("In SetupRoles:getRoles():", err);
          this.roles_state.set('error');
        }
      });
  }
  openNewUserDialog(){
    const ref = this.dialog.open(AddEditUserDialogComponent, {
      width: '920px',
      maxWidth: '95vw',
      data:{
        mode: 'create',
        roles: this.roles,
      },
    });

    ref.afterClosed().subscribe(result => {
      console.log('The dialog was closed');
      if (!result) return;
    })
  }
  openEditUserDialog(row:any){
    console.log("In openRoleRowDialog():",row);
    const ref = this.dialog.open(AddEditUserDialogComponent, {
      width: '920px',
      maxWidth: '95vw',
      data:{
        mode: 'edit',
        user: row,
        roles: this.roles(),
      },
    });
    ref.afterClosed().subscribe(result => {
      console.log('The dialog was closed');
      if (!result) return;
    });
  }
}
function processRoles(roles:Array<Role>):Role[] {
  let result: Role[] = []
  for (const role of roles) {
    const new_role: Role = {
      id: role.id,
      name: role.name,
      description: role.description,
      active: role.active,
      last_modified_by: role.last_modified_by,
      last_modified_on: role.last_modified_on
    };
    result.push(new_role);
  }
  return result;
}
