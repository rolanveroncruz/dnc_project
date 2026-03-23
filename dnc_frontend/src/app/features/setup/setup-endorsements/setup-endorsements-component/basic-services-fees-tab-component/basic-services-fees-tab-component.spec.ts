import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BasicServicesFeesTabComponent } from './basic-services-fees-tab-component';

describe('BasicServicesFeesTabComponent', () => {
  let component: BasicServicesFeesTabComponent;
  let fixture: ComponentFixture<BasicServicesFeesTabComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BasicServicesFeesTabComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(BasicServicesFeesTabComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
