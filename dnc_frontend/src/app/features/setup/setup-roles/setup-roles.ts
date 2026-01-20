import {Component, DestroyRef, inject, OnInit, signal } from '@angular/core';
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';
import {MatTab, MatTabGroup} from '@angular/material/tabs';


import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {ActivatedRoute, ParamMap, Router} from '@angular/router';
import {RolesTabComponent} from './roles-tab-component/roles-tab-component';
import {RolePermissionsTabComponent} from './role-permissions-tab-component/role-permissions-tab-component';


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
  standalone: true
})

export class SetupRoles implements OnInit {
  private router = inject(Router);
  private route = inject(ActivatedRoute);

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
    }).then();
  }

  private destroyRef = inject(DestroyRef);

  constructor(){}

  ngOnInit(): void {
    this.route.queryParamMap
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe((params: ParamMap) => {
        const tab = (params.get('tab') || 'roles').toLowerCase();
        this.selectedTabIndex.set(tab === 'permissions' ? 1 : 0);
      })
  }




}

