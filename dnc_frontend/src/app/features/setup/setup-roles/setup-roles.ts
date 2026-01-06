import {Component, DestroyRef, inject, OnInit, signal, computed,} from '@angular/core';
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';
import {MatTab, MatTabGroup} from '@angular/material/tabs';


import {
  ModifiedRolePermission,
  Role,
  RolePermission,
  RolesAndPermissionsService
} from '../../../api_services/roles-and-permissions-service';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {ActivatedRoute, ParamMap, Router} from '@angular/router';
import {RolesTabComponent} from './roles-tab-component/roles-tab-component';
import {RolePermissionsTabComponent} from './role-permissions-tab-component/role-permissions-tab-component';
import {DataObject, DataObjectsService} from '../../../api_services/data-objects-service';

type LoadState = 'loading' | 'loaded' | 'error';

@Component({
  selector: 'app-setup-roles',
  imports: [
    MatCard,
    MatCardHeader,
    MatCardContent,
    MatTabGroup,
    MatTab,
    MatCardTitle,
    MatCardSubtitle,
    RolesTabComponent,
    RolePermissionsTabComponent
  ],
  templateUrl: './setup-roles.html',
  styleUrl: './setup-roles.scss',
})

export class SetupRoles implements OnInit {
  private router = inject(Router);
  private route = inject(ActivatedRoute);

  roles_state = signal<LoadState>('loading');
  roles = signal<Role[] | null>(null);
  data_objects = signal<DataObject[]|null>(null);

  role_permissions_state = signal<LoadState>('loading');
  role_permissions_raw = signal<RolePermission[] | null>(null);
  modified_role_permissions = computed(()=>this.convertToRolePermissions(this.role_permissions_raw()))


  roles_errorMsg = signal<string | null>(null);
  role_permissions_errorMsg = signal<string | null>(null);

  selectedTabIndex = signal<number>(0);

  onTabIndexChange(index: number) {
    console.log("In onTabIndexChange():", index);
    this.selectedTabIndex.set(index);
    const tab = index === 1 ? 'permissions' : 'roles';
    this.router.navigate([], {
      relativeTo: this.route,
      queryParams: {tab},
      queryParamsHandling: 'merge',
      replaceUrl: true
    });
  }

  private destroyRef = inject(DestroyRef);

  constructor(
    private roles_and_permission_Service: RolesAndPermissionsService,
    private dataObjectsService: DataObjectsService,) {
  }

  ngOnInit(): void {
    this.route.queryParamMap
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe((params: ParamMap) => {
        const tab = (params.get('tab') || 'roles').toLowerCase();
        this.selectedTabIndex.set(tab === 'permissions' ? 1 : 0);
      })
    this.load();
  }

  load() {
    this.roles_state.set('loading');
    this.roles_errorMsg.set(null)
    this.load_roles();
    this.load_data_object();
    this.load_role_permissions();
    console.log("In load(), data objects:",this.data_objects());
  }

  load_roles() {
    this.roles_and_permission_Service.getRoles()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: (res) => {
          this.roles.set(processRoles(res.items));
          this.roles_state.set('loaded');
          console.log("In load(), roles:",this.roles());
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

function processRoles(roles:Array<Role>):Role[]{
  let result:Role[] = []
  for(const role of roles) {
    const new_role:Role = {
      id: role.id,
      name: role.name,
      description: role.description,
      active: role.active,
      last_modified_by: role.last_modified_by,
      last_modified_on: role.last_modified_on
    };
    result.push(new_role);
  }
  console.log("In processRoles():",result);
  return result;
}
