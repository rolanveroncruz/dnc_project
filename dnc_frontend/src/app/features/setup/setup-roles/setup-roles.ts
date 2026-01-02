import {Component, DestroyRef, inject, OnInit, signal} from '@angular/core';
import {MatCard, MatCardContent, MatCardHeader, MatCardTitle} from '@angular/material/card';
import {MatTab, MatTabGroup} from '@angular/material/tabs';
import {User} from '../../../api_services/user-service';
import {Role, RolePermission, RolesAndPermissionsService} from '../../../api_services/roles-and-permissions-service';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';

type LoadState = 'loading' | 'loaded' | 'error';
@Component({
  selector: 'app-setup-roles',
  imports: [
    MatCard,
    MatCardHeader,
    MatCardContent,
    MatTabGroup,
    MatTab,
    GenericDataTableComponent
  ],
  templateUrl: './setup-roles.html',
  styleUrl: './setup-roles.scss',
})
export class SetupRoles implements OnInit {
  roles_state = signal<LoadState>('loading');
  roles = signal<Role[] | null>(null);

  role_permissions_state = signal<LoadState>('loading');
  role_permissions = signal<RolePermission[] | null>(null);

  roles_errorMsg = signal<string | null>(null);
  role_permissions_errorMsg = signal<string | null>(null);

  private destroyRef = inject(DestroyRef);
  roleColumns: TableColumn[] = [
    {key: 'id', label: 'ID'},
    {key: 'name', label: 'Name'},
    {key: 'last_modified_by', label: 'Last Modified By'},
    {key: 'last_modified_on', label: 'Last Modified On'},
  ];
  role_permissionsColumns: TableColumn[] = [
    {key: 'id', label: 'ID'},
    {key: 'role', label: 'Role'},
    {key: 'object', label: 'Object'},
    {key: 'action', label: 'Action'},
    {key: 'last_modified_by', label: 'Last Modified By'},
    {key: 'last_modified_on', label: 'Last Modified On'},
  ];

  constructor(private roles_and_permission_Service:RolesAndPermissionsService) {}

  ngOnInit(): void {
    this.load();
    }

    load(){
      this.roles_state.set('loading');
      this.roles_errorMsg.set(null)
      this.roles_and_permission_Service.getRoles()
        .pipe(takeUntilDestroyed(this.destroyRef))
        .subscribe({
          next: (res) => {
            this.roles.set(res.items);
            this.roles_state.set('loaded');
          },
          error: (err) => {
            console.log("In SetupRoles:getRoles():", err);
            this.roles_errorMsg.set("Failed to load clinic capabilities");
            this.roles_state.set('error');
          }
        });

      this.roles_and_permission_Service.getRolePermissions()
        .pipe(takeUntilDestroyed(this.destroyRef))
        .subscribe({
          next: (res) => {
            this.role_permissions.set(res.items);
            this.role_permissions_state.set('loaded');
          },
          error: (err) => {
            console.log("In SetupRoles():getRolePermissions():",err);
            this.role_permissions_errorMsg.set("Failed to load clinic capabilities");
            this.role_permissions_state.set('error');
          }
        });

    }
}
