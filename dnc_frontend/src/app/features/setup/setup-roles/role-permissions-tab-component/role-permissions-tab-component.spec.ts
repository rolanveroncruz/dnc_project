import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RolePermissionsTabComponent } from './role-permissions-tab-component';

describe('RolePermissionsTabComponent', () => {
  let component: RolePermissionsTabComponent;
  let fixture: ComponentFixture<RolePermissionsTabComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [RolePermissionsTabComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(RolePermissionsTabComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
