import {Component, DestroyRef, inject, OnInit, signal, computed} from '@angular/core';
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';
import {MatTab, MatTabGroup} from '@angular/material/tabs';
import {Role, RolePermission, RolesAndPermissionsService} from '../../../api_services/roles-and-permissions-service';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {AddEditDentalServices} from '../dental-services/add-edit-dental-services/add-edit-dental-services';
import {MatDialog} from '@angular/material/dialog';
import {AddEditRoles} from './add-edit-roles/add-edit-roles';
import {AddEditRolePermissions} from './add-edit-role-permissions/add-edit-role-permissions';

type LoadState = 'loading' | 'loaded' | 'error';
interface ModifiedRolePermission{
  id:number;
  role:string;
  object:string;
  action:string[];
  last_modified_by:string;
  last_modified_on:Date;
}

@Component({
  selector: 'app-setup-roles',
  imports: [
    MatCard,
    MatCardHeader,
    MatCardContent,
    MatTabGroup,
    MatTab,
    GenericDataTableComponent,
    MatCardTitle,
    MatCardSubtitle
  ],
  templateUrl: './setup-roles.html',
  styleUrl: './setup-roles.scss',
})

export class SetupRoles implements OnInit {
  roles_state = signal<LoadState>('loading');
  roles = signal<Role[] | null>(null);

  role_permissions_state = signal<LoadState>('loading');
  role_permissions_raw = signal<RolePermission[] | null>(null);

  role_permissions = computed(()=> this.convertToRolePermissions(this.role_permissions_raw()))

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
    {key: 'action', label: 'Action', cellTemplateKey: 'chips'},
    {key: 'last_modified_by', label: 'Last Modified By'},
    {key: 'last_modified_on', label: 'Last Modified On', cellTemplateKey: 'date'},
  ];
  private dialog = inject(MatDialog);

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
            this.roles.set(processRoles(res.items));
            this.roles_state.set('loaded');
            console.log("In SetupRoles:getRoles():",this.roles());
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
            this.role_permissions_raw.set(res.items);
            this.role_permissions_state.set('loaded');
            console.log("In SetupRoles():getRolePermissions():",this.role_permissions());
          },
          error: (err) => {
            console.log("In SetupRoles():getRolePermissions():",err);
            this.role_permissions_errorMsg.set("Failed to load clinic capabilities");
            this.role_permissions_state.set('error');
          }
        });
    }
    convertToRolePermissions(rows:Array<RolePermission> | null) : Array<ModifiedRolePermission> | null {
      if(rows==null) return null;
      const map = new Map<string, ModifiedRolePermission>();

      for( const rp of rows){
        const key = `${String(rp.role)}\u0000${String(rp.object)}`;
        const entry = map.get(key);
        if(entry){
          entry.action.push(rp.action);
          const l:[string, Date] | undefined = laterOf(entry, rp);
          if (l){
            entry.last_modified_by = l[0];
            entry.last_modified_on = l[1];
          }
        }else{
          map.set(key, {id:rp.id, role:rp.role, object:rp.object, action:[rp.action], last_modified_by:rp.last_modified_by, last_modified_on:rp.last_modified_on});
        }
      }
      return Array.from(map.values());
    }

  openRoleRowDialog(row:any){
    console.log("In openRoleRowDialog():",row);
    const ref = this.dialog.open(AddEditRoles, {
      data:row,
      width: '720px',
      maxWidth: '95vw',
      disableClose : false,
    });
    ref.afterClosed().subscribe(result => {
      console.log('The dialog was closed');
      if (!result) return;
    });
  }

  openRolePermissionRowDialog(row:any){
    console.log("In openRoleRowDialog():",row);
    const ref = this.dialog.open(AddEditRolePermissions, {
      data:row,
      width: '720px',
      maxWidth: '95vw',
      disableClose : false,
    });
    ref.afterClosed().subscribe(result => {
      console.log('The dialog was closed');
      if (!result) return;
    });
  }
}
function toValidDate(x: unknown): Date | undefined {
  const d =
    x instanceof Date ? x :
      (typeof x === 'string' || typeof x === 'number') ? new Date(x) :
        undefined;

  return d && !Number.isNaN(d.getTime()) ? d : undefined;
}

function laterOf(
  a: ModifiedRolePermission,
  b: RolePermission
): [string, Date] | undefined {
  const da = toValidDate(a.last_modified_on);
  const db = toValidDate(b.last_modified_on);

  if (!da) return db ? [b.last_modified_by, db] : undefined;
  if (!db) return [a.last_modified_by, da];

  return da.getTime() > db.getTime()
    ? [a.last_modified_by, da]
    : [b.last_modified_by, db];
}

function processRoles(roles:Array<Role>):Role[]{
  let result:Role[] = []
  for(const role of roles) {
    const new_role:Role = {
      id: role.id,
      name: role.name,
      active: role.active,
      last_modified_by: role.last_modified_by,
      last_modified_on: role.last_modified_on
    };
    result.push(new_role);
  }
  console.log("In processRoles():",result);
  return result;
}
