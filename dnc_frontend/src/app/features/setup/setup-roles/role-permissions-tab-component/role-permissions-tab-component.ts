import {Component, computed, DestroyRef, inject,  OnInit, signal,} from '@angular/core';
import {
  GenericDataTableComponent
} from '../../../../components/generic-data-table-component/generic-data-table-component';
import {
  ModifiedRolePermission, Role, RolePermission, RolesAndPermissionsService,
} from '../../../../api_services/roles-and-permissions-service';
import {DataObject, DataObjectsService} from '../../../../api_services/data-objects-service';
import {MatDialog} from '@angular/material/dialog';
import {TableColumn} from '../../../../components/generic-data-table-component/table-interfaces';
import {AddEditRolePermissionsDialogComponent} from '../add-edit-role-permissions/add-edit-role-permissions.dialog';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';


type LoadState = 'loading' | 'loaded' | 'error';

@Component({
  selector: 'app-role-permissions-tab-component',
  imports: [
    GenericDataTableComponent
  ],
  templateUrl: './role-permissions-tab-component.html',
  styleUrl: './role-permissions-tab-component.scss',
})
export class RolePermissionsTabComponent implements OnInit{
  role_permissions_state = signal<LoadState>('loading');
  role_permissions_raw = signal<RolePermission[] | null>(null);
  role_permissions_errorMsg = signal<string | null>(null);

  modified_role_permissions = computed(()=>this.convertToRolePermissions(this.role_permissions_raw()))

  data_objects = signal<DataObject[]|null>(null);
  roles = signal<Role[]|null>(null);
  roles_state = signal<LoadState>('loading');

  private destroyRef = inject(DestroyRef);
  constructor(
    private dataObjectsService:DataObjectsService,
    private roles_and_permission_Service:RolesAndPermissionsService,
    ){
  }

  ngOnInit(): void {
    this.load_role_permissions();
    this.load_roles();
    this.load_data_object();
  }

  load_roles() {
    this.roles_state.set('loading');
    this.roles_and_permission_Service.getRoles()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: (res) => {
          this.roles.set(processRoles(res.items));
          this.roles_state.set('loaded');
          console.log("In RolePermissionsTabComponent:load_roles():", this.roles());
        },
        error: (err) => {
          console.log("In SetupRoles:getRoles():", err);
          this.roles_state.set('error');
        }
      });
  }
  load_data_object(){
    this.dataObjectsService.getDataObjects()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: (res) => {
          this.data_objects.set(res.items)
          console.log("In load(), data_objects:",this.data_objects());
        },
        error: (err)=>{
          console.log(err)
        }
      });
  }

  load_role_permissions() {
    this.roles_and_permission_Service.getRolePermissions()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: (res) => {
          this.role_permissions_raw.set(res.items);
          this.role_permissions_state.set('loaded');
          console.log("In SetupRoles():getRolePermissions():", this.role_permissions_raw());
        },
        error: (err) => {
          console.log("In SetupRoles():getRolePermissions():", err);
          this.role_permissions_errorMsg.set("Failed to load clinic capabilities");
          this.role_permissions_state.set('error');
        }
      });

  }
  role_permissionsColumns: TableColumn[] = [
    {key: 'id', label: 'ID'},
    {key: 'role_name', label: 'Role'},
    {key: 'object_name', label: 'Object'},
    {key: 'actions', label: 'Action', cellTemplateKey: 'chips'},
    {key: 'last_modified_by', label: 'Last Modified By'},
    {key: 'last_modified_on', label: 'Last Modified On', cellTemplateKey: 'datetime'},
  ];

  private dialog = inject(MatDialog);
  openRolePermissionRowDialog(row:ModifiedRolePermission){
    console.log("In openRoleRowDialog():",row);
    const ref = this.dialog.open(AddEditRolePermissionsDialogComponent, {
      autoFocus: false,
      data:{
        mode: 'edit',
        row,
        roles:this.roles(),
        objects: this.data_objects(),
      },
      width: '720px',
      maxWidth: '95vw',
      maxHeight: '90vh',
      disableClose : true,
    });
    ref.afterClosed().subscribe(result => {
      console.log('The dialog was closed');
      if (!result) return;
    });
  }
  convertToRolePermissions(rows: Array<RolePermission> | null): Array<ModifiedRolePermission> | null {
    if (rows == null) return null;
    const map = new Map<string, ModifiedRolePermission>();

    for (const rp of rows) {
      const key = `${String(rp.role)}\u0000${String(rp.object)}`;
      const entry = map.get(key);
      if (entry) {
        entry.actions.push(rp.action);
        const l: [string, Date] | undefined = laterOf(entry, rp);
        if (l) {
          entry.last_modified_by = l[0];
          entry.last_modified_on = l[1];
        }
      } else {
        map.set(key, {
          id: rp.id,
          role_id:rp.role_id,
          role_name: rp.role,
          object_id: rp.object_id,
          object_name: rp.object,
          active: rp.active,
          actions: [rp.action],
          last_modified_by: rp.last_modified_by,
          last_modified_on: rp.last_modified_on
        });
      }
    }
    return Array.from(map.values());
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
